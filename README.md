I was paying $22 a month for a resume service subscription featuring a web app I would use a few times a month to make updates.

I cancelled my subscription to build a personalized resume generator!

- Options / Organization
- > take user input commands
- > write options to file 
- > various commands to set the options individually and a separate command to generate based on updated options/settings
- 

instead of writing to file and requiring incremental writes,
could we make live updates to the document through some form of socket connection???

___
___

TODO

- Handle all command parsing
- Write command results to file according to serialized structs
- Read and deserialize data from file
- Use available data to write sections to pdf
- custom error handling
- write tests

### _Features_

> Write one of the starter templates and then edit in something like Acrobat.