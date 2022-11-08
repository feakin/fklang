///
#[cfg(test)]
mod tests {
  use crate::deconstruct::model_builder::ModelBuilder;

  #[test]
  fn it_works() {
    let code_file = ModelBuilder::by_str(r#"
@RestController
@Transactional

public class ApplicationController {
  @PostMapping("/client/manager")
	public ResponseEntity<AccountAccessResource> addAccountManager(@RequestBody final AddAccountManagerCommand command,
			@ApiParam(hidden = true) final HttpMethod method, final WebRequest request) {

		  final AccountAccessResource result = new AccountAccessResource();
			return new ResponseEntity<>(result, HttpStatus.CREATED);
  }
}
    "#);

    println!("{:?}", code_file);
  }
}
