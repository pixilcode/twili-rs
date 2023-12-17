use super::{
	Config,
	traits::{
		Presenter,
		TaskDao
	}
};

pub struct InteractConfig;

pub fn run<P, M>(config: &mut Config<P, M, InteractConfig>)
where
	P: Presenter,
	M: TaskDao,
{
	unimplemented!()
}