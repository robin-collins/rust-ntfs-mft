# To Do

## Suggestions

- [ ] Error Handling Improvements: The current error handling in main.rs uses eprintln! to print errors to standard error and then continues or breaks the loop. It would be better to collect these errors and return them as part of the Result, so that the caller of main can handle them appropriately.
- [ ] Logging: Instead of using eprintln!, integrate a logging framework like log and env_logger to provide more control over logging levels and outputs.
- [ ] Configuration Validation: Add validation for the configuration settings loaded from Config::new() to ensure that all required settings are present and valid before proceeding.
- [ ] Progress Feedback: Provide feedback on the progress of reading and parsing MFT entries, especially if the volume is large and the process takes a significant amount of time.
- [ ] Graceful Shutdown: Implement a way to gracefully shut down the process, perhaps by catching a signal, so that the program can exit cleanly if needed.
- [ ] Parallel Processing: Consider using parallel processing to read and parse MFT entries to take advantage of multi-core processors and improve performance.
- [ ] Database Transaction Management: Ensure that database interactions are wrapped in transactions to maintain data integrity, especially in the case of errors or interruptions.
- [ ] Limiting Reads: Introduce a mechanism to limit the number of MFT entries read in one go, to avoid excessive memory usage.
- [ ] Testing: Add unit tests for the various components of the system to ensure reliability and ease future maintenance.
- [ ] Documentation: Include code comments and update the documentation to reflect any changes made to the codebase.

## Done

06/12/2023
Please note that the main.rs assumes the existence of certain methods like

- [x] start_transaction
- [x] store_data
- [x] commit on the DatabaseInterface
- [x] as well as the env_logger crate being added to Cargo.toml.

Implement these methods and ensure the necessary dependencies are included in your project.
