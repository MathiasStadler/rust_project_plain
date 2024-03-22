# rust project plain template

- [rust project plain template](#rust-project-plain-template)
  - [environment](#environment)
    - [this sample works on](#this-sample-works-on)
  - [init](#init)
    - [set and check env variable](#set-and-check-env-variable)
      - [set variable](#set-variable)
      - [check variable is set](#check-variable-is-set)
      - [check project is **NOT** available](#check-project-is-not-available)
    - [things you need](#things-you-need)
      - [shfmt](#shfmt)
        - [install](#install)
        - [apply this tool](#apply-this-tool)
  
<a id="rust-project-plain-template"></a>
  
## environment

<a id="environment"></a>

### this sample works on

```bash
cat /etc/os-release 
PRETTY_NAME="Ubuntu 22.04.4 LTS"
NAME="Ubuntu"
VERSION_ID="22.04"
VERSION="22.04.4 LTS (Jammy Jellyfish)"
VERSION_CODENAME=jammy
ID=ubuntu
ID_LIKE=debian
HOME_URL="https://www.ubuntu.com/"
SUPPORT_URL="https://help.ubuntu.com/"
BUG_REPORT_URL="https://bugs.launchpad.net/ubuntu/"
PRIVACY_POLICY_URL="https://www.ubuntu.com/legal/terms-and-policies/privacy-policy"
UBUNTU_CODENAME=jammy
```

## init

### set and check env variable

#### set variable

```bash
export RUST_PROJECT = "rust_project_plain" 
echo $RUST_PROJECT

```

[toc](#rust-project-plain-template)

#### check variable is set

[FROM HERE](https://stackoverflow.com/questions/3601515/how-to-check-if-a-variable-is-set-in-bash)

<div style="background-color:rgba(250, 230, 7, 1);">
> bash script trap
> https://www.redhat.com/sysadmin/bash-error-handling
> https://opensource.com/article/20/6/bash-trap

</div>

<div style="background-color:rgba(250, 230, 7, 1);">
> this is a note 1
>
> the is is still the note
</div>

```bash




```

#### check project is **NOT** available

1. create check script

```bash
cd /tmp
<<EOF>> check_project_dir_if_not_available.sh cat
if [[ -d "${RUST_PROJECT}" && ! -L "${RUST_PROJECT}" ]] ; then
echo "project dir ${RUST_PROJECT} available. "; 
echo " =>  Delete it or rename your project !!!"; 
else 
echo "Directory not available";
fi
EOF

```

[toc](#rust-project-plain-template)

### things you need

#### shfmt

##### install

```bash
apt update && \
apt install shfmt
```

[toc](#rust-project-plain-template)

##### apply this tool

```bash
# shfmt <file>
# shfmt -i uint   indent: 0 for tabs (default), >0 for number of spaces
# shfmt -w write result to file instead of stdout
# shfmt -s simplify the code

shfmt  -i 4 -w -s tmp_file.sh

```

[toc](#rust-project-plain-template)

[toc](#rust-project-plain-template)
