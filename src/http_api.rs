use hyper;
use hyper::header::{Accept, ContentType, Headers, qitem};
use hyper::status::StatusCode;
use protobuf;
use protobuf::error::ProtobufError;
use std::io::{Read, BufRead, BufReader};
use std::thread::spawn;

const PROTOBUF: &'static str = "application/x-protobuf";

header! { (MesosStreamId, "Mesos-Stream-Id") => [String] }

lazy_static! {
    static ref AcceptProtobuf: Accept = Accept(vec![qitem(PROTOBUF.parse().unwrap())]);
    static ref ContentTypeProtobuf: ContentType = ContentType(PROTOBUF.parse().unwrap());
}

#[derive(Debug)]
pub enum Error {
    Io(hyper::Error),
    Status(StatusCode, String),
    Serialization(ProtobufError)
}

pub type Result<T> = ::std::result::Result<T, Error>;

pub struct HttpApi {
    http_client: hyper::Client,
    endpoint: String,
    mesos_stream_id: Option<MesosStreamId>
}

pub trait HttpHandler<Event: protobuf::MessageStatic> {
    fn on_event(&self, event: Event);
    fn on_error(&self, error: Error);
}

impl HttpApi {
    pub fn new(endpoint: &str) -> Result<Self> {
        let http_client = hyper::Client::new();

        Ok(HttpApi{http_client: http_client, endpoint: endpoint.to_string(), mesos_stream_id: None})
    }

    pub fn send(&self,
                to: &str,
                message: &protobuf::Message) -> Result<()> {
        match self.send_internal(to, message) {
            Ok(hyper::client::Response{status: StatusCode::Accepted, ..}) => Ok(()),
            Ok(resp) => Err(Error::Status(resp.status, format!("{:?}", resp))),
            Err(e) => Err(e)
        }
    }

    fn send_internal(&self,
                     to: &str,
                     message: &protobuf::Message) -> Result<hyper::client::Response> {

        //info!("Fields {:?}", message.descriptor().fields().iter().map(|f| f.name()).collect::<Vec<&str>>());
        //info!("Sending {:?}", message.descriptor().field_by_name("field_type").get_enum(message).name());

        let mut url = "http://".to_string();
        url.push_str(to);
        url.push_str(&self.endpoint);

        let mut headers = Headers::new();
        headers.set(AcceptProtobuf.clone());
        headers.set(ContentTypeProtobuf.clone());
        if let Some(ref mesos_stream_id) = self.mesos_stream_id {
            headers.set(mesos_stream_id.clone());
        }

        let data = try!(message.write_to_bytes().map_err(Error::Serialization));

        self.http_client.post(&url)
                        .headers(headers)
                        .body(&data[..])
                        .send()
                        .map_err(Error::Io)
    }

    pub fn subscribe<Subscribe, Event, Handler>(&mut self,
                                                to: &str,
                                                subscribe: Subscribe,
                                                handler: Handler)-> Result<()>
    where Subscribe: protobuf::MessageStatic,
          Event: protobuf::MessageStatic,
          Handler: HttpHandler<Event> + Send + 'static {

        let response = try!(self.send_internal(to, &subscribe));

        if response.status == StatusCode::Ok {            
            if let Some(mesos_stream_id) = response.headers.get::<MesosStreamId>() {
                info!("Mesos-Stream-Id {}", mesos_stream_id);
                self.mesos_stream_id = Some(mesos_stream_id.clone());
            }            
            spawn(move || {
                let mut stream = BufReader::new(response);
                loop {
                    match stream.read_message::<Event>() {
                        Ok(event) => handler.on_event(event),
                        Err(err) => handler.on_error(Error::Serialization(err))
                    }
                }
            });
            Ok(())
        } else {
            Err(Error::Status(response.status, format!("{:?}", response)))
        }
    }
}

trait ChunkedEncodedProtobufReader {
    fn read_message<M: protobuf::Message + protobuf::MessageStatic>(&mut self) -> protobuf::ProtobufResult<M>;
}

impl <R: BufRead> ChunkedEncodedProtobufReader for R {
    fn read_message<M: protobuf::Message + protobuf::MessageStatic>(&mut self) -> protobuf::ProtobufResult<M> {
        let len_str = try!(self.lines().next().expect("LINE").map_err(ProtobufError::IoError));
        let len: u64 = try!(len_str.parse().map_err(|_|ProtobufError::WireError("".to_string())));
        trace!("protobuf.len = {:?}", len);
        protobuf::parse_from_reader(&mut self.take(len))
    }
}
