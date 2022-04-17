use grpc::{ServerHandlerContext,ServerRequestSingle,ServerResponseUnarySink};
use chat_grpc::*;
use chat::*;

mod chat;
mod chat_grpc;
struct MyChat;

impl Chat for MyChat {
    fn poop(
        &self,
        _: ServerHandlerContext,
        req: ServerRequestSingle<ChatRequest>,
        resp: ServerResponseUnarySink<ChatResponse>,
    ) -> grpc::Result<()> {
        let mut r = ChatResponse::new();
        let content = if req.message.get_name().is_empty() {
            "world"
        } else {
            req.message.get_name()
        };

        println!("greeting request from {}", content);
        r.set_content(format!("Hello {}", content));
        resp.finish(r)
    }
}

fn main() {
    let port = 50051;

    let mut server = grpc::ServerBuilder::new_plain();

    server.http.set_port(port);

    server.add_service(ChatServer::new_service_def(MyChat));

    let _server = server.build().expect("server");
    println!(
        "greeter server started on port {}",
        port,
    );

    loop {
        std::thread::park();
    }
}