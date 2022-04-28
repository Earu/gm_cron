#![feature(c_unwind)]

use std::cell::RefCell;
use job_scheduler::{JobScheduler, Job, Schedule, Uuid};
use closure::closure;

#[macro_use]
extern crate gmod;
extern crate job_scheduler;

thread_local! {
    static SCHEDULER: std::cell::RefCell<JobScheduler<'static>> = RefCell::new(JobScheduler::new());
}

struct CronJob {
	id: String,
	reference: i32,
}

impl CronJob {
	unsafe fn push(&self, lua: gmod::lua::State) {
		lua.new_table();

		lua.push_string(&self.id);
		lua.set_field(-2, lua_string!("Id"));

		lua.push_integer(self.reference as isize);
		lua.set_field(-2, lua_string!("Reference"));

		lua.push_string("CronJob");
		lua.set_field(-2, lua_string!("__type"));

		lua.push_function(CronJob::remove);
		lua.set_field(-2, lua_string!("Remove"));
	}

	unsafe fn check_cron_job(lua: gmod::lua::State, index: i32) -> CronJob {
		if !lua.is_table(index) {
			lua.error("Invalid arg #1 expected CronJob");
		}

		lua.get_field(index, lua_string!("__type"));
		let tbl_type = lua.get_string(-1);
		lua.pop();

		match tbl_type {
			Some(tbl_type) if tbl_type.as_ref() != "CronJob" => lua.error("Invalid arg #1 expected CronJob"),
			None => lua.error("Invalid arg #1 expected CronJob"),
			_ => {
				lua.get_field(index, lua_string!("Id"));
				let id = lua.get_string(-1).as_ref().unwrap().to_string();
				lua.pop();

				lua.get_field(index, lua_string!("Reference"));
				let reference = lua.check_integer(-1) as i32;
				lua.pop();

				CronJob {
					id,
					reference,
				}
			},
		}
	}

	#[lua_function]
	unsafe fn remove(lua: gmod::lua::State) -> i32 {
		let job = CronJob::check_cron_job(lua, 1);
		lua.dereference(job.reference);

		SCHEDULER.with(|scheduler| {
			if let Ok(job_id) = Uuid::parse_str(job.id.as_ref()) {
				scheduler.borrow_mut().remove(job_id);
			}
		});

		0
	}
}

#[lua_function]
unsafe fn cron_job(lua: gmod::lua::State) -> i32 {
	let input = lua.check_string(1);
	lua.check_function(2);
	lua.push_value(2);
	let on_job_reference = lua.reference();

	match input.parse::<Schedule>() {
		Ok(schedule) => {
			SCHEDULER.with(|scheduler| {
				let mut scheduler = scheduler.borrow_mut();
				let job = Job::new(schedule, closure!(move lua, move on_job_reference, || {
					lua.from_reference(on_job_reference);
					lua.call(0, 0);
				}));

				let id = scheduler.add(job).to_string();
				CronJob {
					id,
					reference: on_job_reference,
				}.push(lua);
			});
		},
		Err(e) => lua.error(format!("{}", e)),
	}

	1
}

#[lua_function]
unsafe fn scheduler_tick(_: gmod::lua::State) -> i32 {
	SCHEDULER.with(|scheduler| {
		scheduler.borrow_mut().tick();
	});

	0
}

unsafe fn initialize_scheduler(lua: gmod::lua::State)
{
	lua.get_global(lua_string!("hook"));
		lua.get_field(-1, lua_string!("Add"));
			lua.push_string("Think");
			lua.push_string("__CronScheduler");
			lua.push_function(scheduler_tick);
		lua.call(3, 0);
	lua.pop_n(2);
}

#[gmod13_open]
unsafe fn gmod13_open(lua: gmod::lua::State) -> i32 {
	initialize_scheduler(lua);

	lua.push_function(cron_job);
	lua.set_global(lua_string!("CronJob"));

    0
}

#[gmod13_close]
unsafe fn gmod13_close(_: gmod::lua::State) -> i32 {
    0
}