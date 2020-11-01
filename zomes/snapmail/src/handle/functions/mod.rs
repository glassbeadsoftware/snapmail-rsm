mod ping_agent;

mod get_handle;
mod get_my_handle;
mod set_handle;

//mod get_my_handle_history;
//mod find_agent;
mod get_all_handles;

pub use self::{
    ping_agent::*,

    get_handle::*,
    get_my_handle::*,
    set_handle::*,

   // get_my_handle_history::*,
    //find_agent::*,
    get_all_handles::*,
};
