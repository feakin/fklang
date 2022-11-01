use fkl_parser::mir::ContextMap;
use futures::executor::block_on;
use crate::mock::stub_server::feakin_rocket;

pub(crate) async fn mock_server_runner(mir: &ContextMap) {
  let _ = block_on(async { feakin_rocket(mir).launch() }).await;
}
