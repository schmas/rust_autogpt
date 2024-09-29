mod ai_functions;
mod apis;
mod helpers;
mod models;

use crate::helpers::command_line::get_user_response;

fn main() {
    let usr_request = get_user_response("What webserver are we building today?");

    dbg!(usr_request);
}
