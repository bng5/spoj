#!/usr/bin/php
<?php

while (($num = fgets(STDIN)) !== false) {
    $num = trim($num);
    if($num === '42') {
      break;
    }
    echo $num.PHP_EOL;
}

?>
