use jsonrpc_http_server::jsonrpc_core::*;
use jsonrpc_http_server::{AccessControlAllowOrigin, DomainsValidation, RestApi, ServerBuilder};
use std::collections::HashMap;
use std::vec::Vec;
use serde_json;


fn send_transaction(s:& String)->String{
	/*
		如果传入的值是hashmap型 则需要进行切片，将字符串中的中括号 ‘[  ]’去掉
		然后再将字符串转为map，对其进行操作 ，最后返回结果
	*/
	let s0="".to_string();
	let s_send=s0+&s[1..s.len()-1];
	let map:HashMap<String,Value>=serde_json::from_str(&s_send).unwrap();
	let str_from=map.get("from").unwrap().to_string().replace("\""," ");
	let return_from=str_from.trim();
	let str_to=map.get("to").unwrap().to_string().replace("\""," ");
	let return_to=str_to.trim();
	let s=String::from("");
	let str_return=s+return_from+return_to;
	str_return.to_string()
	/*
		如果传入的值是Vec，则直接通过下列语句转为vec 再对其进行操作，然后返回结果
	*/
	//let vec:Vec<String>=serde_json::from_str(&s).unwrap();
}
fn main() {
	env_logger::init();
	let mut io = IoHandler::default();

	io.add_method("say_hello", |_params: Params| Ok(Value::String("hello".to_string())));
	io.add_method("eth_sendTransaction", |_params: Params|{
		let par=serde_json::to_string(&_params);//取出传进来的param值
		let s=match par{
			Ok(i)=>i,
			Err(_e)=>"error".to_string()
		};  //将param值转为字符串并送入函数进行操作
		let res=send_transaction(&s);
		Ok(Value::String(res))            //将函数返回值传出，显示为result
	});


	let server = ServerBuilder::new(io)
		.threads(3)
		.rest_api(RestApi::Unsecure)
		.cors(DomainsValidation::AllowOnly(vec![AccessControlAllowOrigin::Any]))
		.start_http(&"127.0.0.1:3030".parse().unwrap())
		.expect("Unable to start RPC server");

	server.wait();
}
