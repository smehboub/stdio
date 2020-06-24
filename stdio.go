package main

import "os"
import "fmt"

func main() { 
  fmt.Fprintln(os.Stdout, "your stdout string");
  fmt.Fprintln(os.Stderr, "your stderr string");
}
