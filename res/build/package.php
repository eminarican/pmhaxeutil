<?php

function main() {
    if (isPharReadonly()) {
        exitError("Phar is readonly, set phar.readonly to 0 with -dphar.readonly=0!");
    }

    rename("out/src/lib", "build/src");
    rename("build/src", "out/src");

    // Todo: read plugin.yml
    package("Plugin.phar", [], "out");
}

function package(string $name, array $metadata, string $from) {
    if(file_exists($name)){
        @\Phar::unlinkArchive($name);
    }
    
    $phar = new Phar($name);
    $phar->setMetadata($metadata);
    $phar->setStub('<?php echo "PocketMine-MP plugin \n";if(extension_loaded("phar")){$phar = new \Phar(__FILE__);foreach($phar->getMetadata() as $key => $value){echo ucfirst($key).": ".(is_array($value) ? implode(", ", $value):$value)."\n";}} __HALT_COMPILER();');
    $phar->setSignatureAlgorithm(Phar::SHA256);
    $phar->buildFromDirectory("out");
}

function isPharReadonly(): bool {return ini_get("phar.readonly") == 1;}

function exitError(string $message) {
    echo $message . "\n";
    exit(1);
}

function exitOk(string $message) {
    echo $message . "\n";
    exit(0);
}

main();
