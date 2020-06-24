<?php
$stdout = fopen('php://stdout', 'w');
fputs($stdout, "your stdout string.\n");
fclose($stdout); 
$stderr = fopen('php://stderr', 'w');
fputs($stderr, "your stderr string.\n");
fclose($stderr); 
?>
