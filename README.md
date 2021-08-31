# Gazer
A project for the Code Challenge given by the [Anchor](https://github.com/Anchor-Protocol) team

Gazer listens for `new_block` messages over a websocket connection to `observer.terra.dev` and extracts event information about UST borrowed on Anchor Protocol.

## Usage

Please run in an environment with `RUST_LOG=debug` to see everything that is going on.

Because of the bug noted in the Bugs section, I've provided the `example_success.txt` file to show what a successfully extracted set of `Borrower`s looks like.

## Needed improvements

- `Release` profile
- Refactor iterations with adaptors
- Integration tests
- More unit tests
- A webserver presenting the Borrower tally
- Custom `Deserializer` for Observer messages for better error handling
- More idiomatic `Borrower` construction logic

## Structure

Gazer is is made of three parts (two of which actually exist right now) that accomplish the following: 

1. Subscribe to the Terra Observer `new_block` websocket stream. This is accomplished by the `observe` module.
2. Extract borrowing events from the resulting JSON messages, accomplished by the `extract` module.
3. Serve the resulting data. Coming soon (`serve`).

Each of these parts are representing by threads spawn in the `lib::run()` function. Additionally, each of these threads is a keep-alive wrapper for the actual worker threads;
This means in the connection to the Observer is closed unexpectedly, or the an unrecoverable error occurs during extraction, Gazer will just start another working thread and
attempt to continue.

## Models

The `models` module contains the structures used for deserializing the JSON messages from Observer (via `extract` functions), and subsequently converting the `borrow_stable` event data into an idiomatic
representation called `Borrower`. My intent is the save `Borrower` objects to a database table, from which the data can manipulated and presented on a webserver.

## Bugs

I discovered too late that occasionally the transaction blocks in the `new_block` messages will not have a `logs` section. This is a problem because the structs the JSON is
deserialized into assumes it is there. The result is that only messages that, A. have a borrow event, and B. Have no `txs` blocks missing the `logs` section yield a 
tally-able Borrower struct. Because of my lack of knowledge of `serde_json`, I am unsure how to skip these transactions while deserializing. I imagine it might require a custom
Deserializer implementation, which is a little over my head right now.