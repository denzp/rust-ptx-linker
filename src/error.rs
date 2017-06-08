error_chain!{
    errors {
        PathArgumentError(argument: String) {
            description("Expected path")
            display("Expected path, got `{}`", argument)
        }

        NoOutputPathError {
            display("No output path is specified")
        }
    }
}
