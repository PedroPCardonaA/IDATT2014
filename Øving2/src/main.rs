use std::sync::{Arc, RwLock, Mutex, Condvar};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::time::SystemTime;


fn main() {
    let mut workers = Workers::new();
    println!("Number of workers thread = {}", workers.no_threads);
    println!("Number of event Loop = {}", workers.event_loop);
    workers.set_no_threads(10);
    workers.add_tasks(100);
    workers.workers_start();
    workers.generator(10);
    workers.event_loop();
    workers.join();
}


struct Workers{
    no_threads: i32,
    event_loop: i32,
    tasks: Arc<RwLock<Vec<fn()>>>,
    treads: Vec<JoinHandle<()>>,
    events: Arc<Mutex<Vec<fn()>>>,
    pair: Arc<Mutex<(bool, Condvar)>>
}

impl Workers {
 
    fn new() -> Workers{
        Workers { no_threads: (4), event_loop: (1), tasks: (Arc::new(RwLock::new(Vec::new()))), treads:Vec::new(), events:(Arc::new(Mutex::new(Vec::new())))
        , pair: Arc::new(Mutex::new((false, Condvar::new()))) }
    }

    fn set_no_threads(&mut self, no: i32){
        self.no_threads = no;
    }

    fn set_event_loop(&mut self,no: i32){
        self.event_loop = no;
    }

    fn task_a(){
        let id = thread::current().id();
        println!("This is the mega super amazing task A solved in {:?}", id);
    }
    
    fn task_b(){
        let id = thread::current().id();
        println!("This is the Magnificent extreme omega deluxe task B solved in {:?}", id);
    }
    fn task_c(){
        let id = thread::current().id();
        println!("Its me task C and must be solved in series by the epic Event loop {:?}", id)
    }
    fn task_d(){
        let id = thread::current().id();
        println!("Whasaaaaaa im task D and must be solved in series by epic the Event loop {:?}", id)
    }

    fn add_tasks(&mut self, no: i32){

        
        for _x in 0..no{
            let now = SystemTime::now();
            let since_the_epoch = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();
            let nanos = since_the_epoch.subsec_nanos();
            if (nanos/200)%2 ==0{
                self.tasks.write().unwrap().push(Self::task_a)
            } else {
                self.tasks.write().unwrap().push(Self::task_b)
            }
        }

        
        println!("Number of tasks in the list = {}",self.tasks.read().unwrap().len())
    }

    fn workers_start(&mut self){
        for _a in 0..self.no_threads{
            let task_copy = self.tasks.clone();
            self.treads.push(
                thread::spawn(move ||{
                    let id = thread::current().id();
                    println!("{:?} starts!", id);
                    while task_copy.read().unwrap().len() > 0 {
                        let func = task_copy.write().unwrap().remove(0);
                        {
                            thread::sleep(Duration::from_secs(1));
                            func();
                        }
                    }
                    println!("{:?} is done!", id);
                })
            )
        }
    }


    fn generator(&mut self, no:i32){
        let pair_copy = self.pair.clone();
        let copy_events = self.events.clone();
        self.treads.push( thread::spawn(move ||{
            for _a in 0..no{
                let now = SystemTime::now();
                let since_the_epoch = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();
                let nanos = since_the_epoch.subsec_nanos();
                if (nanos/200)%2 ==0{
                    copy_events.lock().unwrap().push(Self::task_c)
                } else {
                    copy_events.lock().unwrap().push(Self::task_d)
                }
                
            }
            let (ref mut value, ref cva) = *pair_copy.lock().unwrap();
            *value = *std::borrow::BorrowMut::borrow_mut(&mut true);
            cva.notify_one();
        }));
        
    }

    fn event_loop(&mut self){
        
        let pair_copy = self.pair.clone();
        let events_copy = self.events.clone();
        self.treads.push( thread::spawn(move ||{
            println!("Event loop is running");

            let (ref mut value, ref cva) = *pair_copy.lock().unwrap();
            while !*value{
                let guard = pair_copy.lock().unwrap();
                cva.wait(guard).unwrap();
            }

            while events_copy.lock().unwrap().len() > 0{
                let event = events_copy.lock().unwrap().remove(0);
                {
                    thread::sleep(Duration::from_secs(1));
                    event();
                }
            }
            println!("Event loop is done");
        }))
    }

    fn join(self){
        
        for t in self.treads{
            let _ = t.join();
        }
    }



}
