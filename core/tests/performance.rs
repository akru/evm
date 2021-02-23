use std::rc::Rc;
use evm_core::{Machine, Capture, ExitSucceed};

macro_rules! ret_test {
	( $name:ident, $code:expr, $data:expr, $ret:expr ) => (
		#[test]
		fn $name() {
			let code = hex::decode($code).unwrap();
			let data = hex::decode($data).unwrap();

			let mut vm = Machine::new(Rc::new(code), Rc::new(data), 1024, 10000, None);
			assert_eq!(vm.run(), Capture::Exit(ExitSucceed::Returned.into()));
			assert_eq!(vm.return_value(), hex::decode($ret).unwrap());
		}
	);
}

ret_test!(
	ackermann31,
	"60e060020a6000350480632839e92814601e57806361047ff414603457005b602a6004356024356047565b8060005260206000f35b603d6004356099565b8060005260206000f35b600082600014605457605e565b8160010190506093565b81600014606957607b565b60756001840360016047565b90506093565b609060018403608c85600186036047565b6047565b90505b92915050565b6000816000148060a95750816001145b60b05760b7565b81905060cf565b60c1600283036099565b60cb600184036099565b0190505b91905056",
	"2839e92800000000000000000000000000000000000000000000000000000000000000030000000000000000000000000000000000000000000000000000000000000001",
	"000000000000000000000000000000000000000000000000000000000000000d"
);

ret_test!(
	ackermann32,
	"60e060020a6000350480632839e92814601e57806361047ff414603457005b602a6004356024356047565b8060005260206000f35b603d6004356099565b8060005260206000f35b600082600014605457605e565b8160010190506093565b81600014606957607b565b60756001840360016047565b90506093565b609060018403608c85600186036047565b6047565b90505b92915050565b6000816000148060a95750816001145b60b05760b7565b81905060cf565b60c1600283036099565b60cb600184036099565b0190505b91905056",
	"2839e92800000000000000000000000000000000000000000000000000000000000000030000000000000000000000000000000000000000000000000000000000000002",
	"000000000000000000000000000000000000000000000000000000000000001d"
);

ret_test!(
	fibonacci10,
	"60e060020a6000350480632839e92814601e57806361047ff414603457005b602a6004356024356047565b8060005260206000f35b603d6004356099565b8060005260206000f35b600082600014605457605e565b8160010190506093565b81600014606957607b565b60756001840360016047565b90506093565b609060018403608c85600186036047565b6047565b90505b92915050565b6000816000148060a95750816001145b60b05760b7565b81905060cf565b60c1600283036099565b60cb600184036099565b0190505b91905056",
	"61047ff4000000000000000000000000000000000000000000000000000000000000000a",
	"0000000000000000000000000000000000000000000000000000000000000037"
);

ret_test!(
	fibonacci16,
	"60e060020a6000350480632839e92814601e57806361047ff414603457005b602a6004356024356047565b8060005260206000f35b603d6004356099565b8060005260206000f35b600082600014605457605e565b8160010190506093565b81600014606957607b565b60756001840360016047565b90506093565b609060018403608c85600186036047565b6047565b90505b92915050565b6000816000148060a95750816001145b60b05760b7565b81905060cf565b60c1600283036099565b60cb600184036099565b0190505b91905056",
	"61047ff40000000000000000000000000000000000000000000000000000000000000010",
	"00000000000000000000000000000000000000000000000000000000000003db"
);
