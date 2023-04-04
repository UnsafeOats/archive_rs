# archive-rs

just a small crate to interact with https://archive.is in the wild.  i reverse-engineered the major parts of the archive.is api and have provided two simple methods of interacting with archive.is from inside your rust code.

# how to use
this crate is designed with simplicity in mind.  most people will want to use the `wait_for_archive` method of the `ArchiveSesh` struct.  this method will simply take a url and some time parameters (for waiting) and then executing the following sequential steps:
  1) if url has already been archived, returns a link to the most recent archived version
  2) if url has not been archived, submit the url to be archived and wait until the archival process is complete
    * there are limitations to this waiting, however.  you must set a wait duration and maximum number of retries, if you hit those max retries it will simply return the link to the in-progress archive

that's it (for the most part).  check the examples folder for a simple crate example showing how to use.

# limitations
it uses tokio for async.  that's it.  if you like a different async runtime, i'm sorry, but no one else will ever see this and i use tokio.
(i'm open to PRs if you want tho)

# license
mit.
