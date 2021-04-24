# rshell

Basic reverse shell written in Rust for Windows.

<pre><code>
USAGE:
    rshell.exe &lthost&gt &ltport&gt

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    &lthost&gt    Remote Host
    &ltport&gt    Remote Port
</pre></code>   
  
### TODO
- Better error handling to escape the current unwrap hell
- In some cases the process is not closed when the server closes the connection
- Implement it over the Blockchain
- Make it AI Driven so it is NextGen
