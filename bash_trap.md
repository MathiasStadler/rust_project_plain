# try out bash trap

- [link](https://citizen428.net/blog/bash-error-handling-with-trap/)

## work folder

```bash
cd /tmp
```

## first script

```bash
<<EOF>> test_bash_trap_1.sh cat
#!/bin/bash

echo "test bash 1.sh";
EOF

bash +x test_bash_1.sh
shfmt -w -i 4 test_bash_trap_1.sh 

```

## script echo script name

```bash
<<EOF> test_bash_trap_1.sh cat
#!/bin/bash
_self="\${0##*/}"
echo "\$_self is started";
echo "Hello from \$_self";
echo "command arg=> \$@";
echo "\$_self is end";

echo "execute \$0";
echo "execute \${0##*/}";
echo "execute => sh +x ./\$_self";
echo "format  => shfmt -w -i 4 ./\$_self";
exit 0;
EOF
```

## script exit on error

```bash
<<EOF> test_bash_trap_1.sh cat
#!/bin/bash
set -Eeuo pipefail
_self="\${0##*/}"
echo "\$_self is started";
echo "Hello from \$_self";
echo "command arg=> \$@";
echo "\$_self is end";

echo "execute \$0";
echo "execute \${0##*/}";
echo "execute => sh +x ./\$_self";
echo "format  => shfmt -w -i 4 ./\$_self";
exit 0;
EOF
```