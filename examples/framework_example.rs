extern crate env_logger;
#[macro_use]
extern crate log;
extern crate mesos;
extern crate proto;
extern crate uuid;

use proto::{CommandInfo, ExecutorID, Filters, FrameworkID, FrameworkInfo,
            MasterInfo, Offer, OfferID, Resource, SlaveID, TaskID, TaskInfo,
            TaskStatus, Value_Scalar, Value_Type};
use mesos::scheduler::Scheduler;
use mesos::scheduler_driver::{SchedulerDriver, MesosSchedulerDriver};

struct MyScheduler;
#[allow(unused_variables)]
impl Scheduler for MyScheduler {
    fn disconnected(&self, driver: &SchedulerDriver) {}

    // Invoked when there is an unrecoverable error in the scheduler or driver.
    fn error(&self, driver: &SchedulerDriver, message: &str) {}

    // Invoked when an executor has exited/terminated.
    fn executor_lost(&self, driver: &SchedulerDriver, executor_id: &ExecutorID, slave_id: &SlaveID, status: isize) {}

    // Invoked when an executor sends a message.
    fn framework_message(&self, driver: &SchedulerDriver, executor_id: &ExecutorID, slave_id: &SlaveID, data: &[u8]) {}

    // Invoked when an offer is no longer valid (e.g., the slave was lost or another framework used resources in the offer).
    fn offer_rescinded(&self, driver: &SchedulerDriver, offer_id: &OfferID) {
        info!("Offer rescinded {:?}", offer_id);
    }

    // Invoked when the scheduler successfully registers with a Mesos master.
    fn registered(&self, driver: &SchedulerDriver, framework_id: &FrameworkID, master_info: &MasterInfo) {
        info!("Registered {:?} {:?}", framework_id, master_info);
    }

    // Invoked when the scheduler re-registers with a newly elected Mesos master.
    fn reregistered(&self, driver: &SchedulerDriver, master_info: &MasterInfo) {}

    // Invoked when resources have been offered to this framework.
    fn resource_offers(&self, driver: &SchedulerDriver, offers: &Vec<Offer>) {
        info!("Resources offered: {:?} offer", offers.len());

        for offer in offers {
            let mut task_id = TaskID::new();
            task_id.set_value(uuid::Uuid::new_v4().to_string());

            let mut resource1 = Resource::new();
            resource1.set_name("cpus".to_string());
            resource1.set_field_type(Value_Type::SCALAR);
            let mut scalar = Value_Scalar::new();
            scalar.set_value(1.0);
            resource1.set_scalar(scalar);

            let mut resource2 = Resource::new();
            resource2.set_name("mem".to_string());
            resource2.set_field_type(Value_Type::SCALAR);
            let mut scalar = Value_Scalar::new();
            scalar.set_value(64.0);
            resource2.set_scalar(scalar);

            let mut command = CommandInfo::new();
            command.set_shell(true);
            command.set_value("echo Hello Mesos!".to_string());

            let mut task = TaskInfo::new();
            task.set_name(format!("task {:?}", task_id));
            task.set_task_id(task_id);
            task.set_slave_id(offer.get_slave_id().clone());
            task.mut_resources().push(resource1);
            task.mut_resources().push(resource2);
            task.set_command(command);

            info!("Launching task {:?}", task.get_task_id());

            driver.launch_tasks(&vec![offer.get_id().clone()], &vec![task], &Filters::new()).unwrap();
        }
    }

    // Invoked when a slave has been determined unreachable (e.g., machine failure, network partition).
    fn slave_lost(&self, driver: &SchedulerDriver, slave_id: &SlaveID) {}

    // Invoked when the status of a task has changed (e.g., a slave is lost and so the task is lost, a task finishes and an executor sends a status update saying so, etc).
    fn status_update(&self, driver: &SchedulerDriver, status: &TaskStatus) {
        info!("Status updated TaskStatus {:?}", status);
    }
}

fn main() {
    env_logger::init().unwrap();

    let scheduler = MyScheduler;
    let master = "zk://localhost:2181/mesos";
    let mut framework = FrameworkInfo::new();

    framework.set_name("rustframework".to_string());
    framework.set_user("bonifaido".to_string());
    framework.set_failover_timeout(60.0);

    let driver = MesosSchedulerDriver::new(scheduler, framework, master).unwrap();
    driver.run();
}