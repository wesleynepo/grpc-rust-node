const PROTO_PATH = __dirname + '/../proto/chat.proto';

const grpc = require('@grpc/grpc-js');
const protoLoader = require('@grpc/proto-loader');

const packageDefinition = protoLoader.loadSync(
    PROTO_PATH,
    {
        keepCase: true,
        longs: String,
        enums: String,
        defaults: true,
        oneofs: true
    }
);

const chat = grpc.loadPackageDefinition(packageDefinition).chat;

function main() {
    const client = new chat.Chat('localhost:50051', grpc.credentials.createInsecure());

    client.poop({name: ''}, (err, response) => {
        if (err) return console.log(err)
        console.log("Hole", response.content)
    })
}


if (require.main === module) {
    main();
  }