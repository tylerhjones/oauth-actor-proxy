use warp::{Filter};
use serde_derive::{Deserialize, Serialize};
use riker::{
    actors::{Actor, Context, Sender, ActorSystem},
};
use riker_patterns::ask::ask;
use futures::{
    executor::block_on,
    future::RemoteHandle,
};

#[derive(Deserialize, Serialize)]
struct VerifyRequest {
    token:String,
}

#[derive(Debug, Clone)]
struct ValidationMsg {
    token:String,
    auth_header:String,
}

#[derive(Default)]
struct Validator;

impl Actor for Validator {
    type Msg = ValidationMsg;


    fn recv(&mut self,
        ctx: &Context<Self::Msg>,
        msg: Self::Msg,
        sender: Sender) {

        println!("Received: {:?}", msg);
        println!("Sender is: {:?}", sender);

        let _res = sender.as_ref()
            .unwrap()
            .try_tell("valid!", Some(ctx.myself().into()));
    }
}

#[tokio::main]
async fn main() {
    let sys = ActorSystem::new().unwrap();

    let validator = sys.sys_actor_of::<Validator>("validator").unwrap();

    let cloned_sys = sys.clone();

    let verify = warp::post()
        .and(warp::path("verify"))
        .and(warp::header::header("Authorization"))
        .and(warp::body::json())
        .map(move |auth_header: String, verify_req: VerifyRequest| {
            let msg = ValidationMsg { 
                token: verify_req.token, 
                auth_header
            };

            let res:RemoteHandle<ValidationMsg> = ask(&cloned_sys, &validator, msg);
            let answer = block_on(res);

            println!("The answer is {:?}", answer);
            format!("bingo!!")
        });

    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let routes = hello.or(verify);

    println!("Starting up...");

    warp::serve(routes)
        .run(([0, 0, 0, 0], 3030))
        .await
}

// API example https://blog.logrocket.com/creating-a-rest-api-in-rust-with-warp/
// Actor example https://github.com/riker-rs/riker/issues/27
// Docker example https://github.com/jayy-lmao/rust-graphql-docker/blob/master/api/Dockerfile