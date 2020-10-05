
# Table of Contents

1.  [What is this?](#orgce73af9)
    1.  [Why only a &rsquo;kind of&rsquo; replacement?](#org3cd6b27)
    2.  [Platform support](#org26cb271)
2.  [Installation](#orge23286a)
    1.  [AUR](#org7f6b4a7)
    2.  [crates.io (rusts package manager)](#org3ac9e43)
    3.  [From the release page](#orgbb4d594)
    4.  [compile manually](#org1720fc7)
3.  [Usage:](#org5353508)
4.  [Security](#orgbb5eab1)

yas - a (kind of) sudo replacement


<a id="orgce73af9"></a>

# What is this?

yas, or &rsquo;yet another sudo&rsquo;, is kind of a sudo replacement, written in rust and without really any configuration options or not needed features. (except for the tui version)
yas also can&rsquo;t run commands as any other user, but root.


<a id="org3cd6b27"></a>

## Why only a &rsquo;kind of&rsquo; replacement?

Yas is only thought for single user systems, not for systems with 3+ users, where some should have more perms, and some less.


<a id="org26cb271"></a>

## Platform support

Has only been tested on linux, for problems please open a issue.
I am pretty sure, that yas won&rsquo;t work on BSD systems, as the password hash file is somewhere else.
It also doesn&rsquo;t work yet on macos, as macos also doesn&rsquo;t have a \`/etc/passwd\`/\`/etc/shadow file\`


<a id="orge23286a"></a>

# Installation


<a id="org7f6b4a7"></a>

## AUR

There is a AUR package for the git version of yas, you can install it with a AUR helper, or just git clone it manually:
Replace yas-git with yas-tui-git for a tui version.

    git clone ssh://aur@aur.archlinux.org/yas-git.git
    cd yas-git
    makepkg -is

or (for yay):

    yay -S yas-git


<a id="org3ac9e43"></a>

## crates.io (rusts package manager)

Get it from crates.io with rusts cargo, by running:

    cargo install yas

NOTE: The suid bit will not be set, you will manually have to set it for yas to work:

    su -c "chown root ~/.cargo/bin/yas && chmod u+s ~/.cargo/bin/yas"


<a id="orgbb4d594"></a>

## From the release page

If you get a libc error when running, you should compile it from source, this will happen espcially often with distros that are debian based.
Or you can get the release from the release page with \`-musl\`, note that there isn&rsquo;t a tui build for that, because ncurses fails to compile. You will have to compile it manually then.

1.  Download a binary
2.  Change the owner to root

    chown root yas

1.  Set the suid bit

    chmod u+s yas

1.  Put it somewhere in your path, such as /usr/bin


<a id="org1720fc7"></a>

## compile manually

1.  Install rust (<https://www.rust-lang.org/>)
2.  Clone the source
3.  build it in release mode, either with tui or not

    git clone https://github.com/alx365/yas.git
    
    cargo build --release # if with tui, add the `--features "tui"` flag
    
    strip target/release/yas

1.  Change the owner to root

    chown root yas

1.  Set the suid bit

    chmod u+s yas


<a id="org5353508"></a>

# Usage:

    yas - execute commands as the root user
    
    usage: yas [-h/--help] [-v/--version ]<command> <arguments for the command, this can be chained infinite>

As one can see, there aren&rsquo;t really any options for yas.
99% of the people who use sudo (or opendoas), just use it with default configuration and the default options.
Who would also need a flag, to edit a file, if nobody ever uses that, and instead just runs \`sudo <editor> <file>\`


<a id="orgbb5eab1"></a>

# Security

> You could have a car that is very safe by just limiting its
> max speed to 20 miles an hour. But I don&rsquo;t want to do that, I want to have
> something that&rsquo;s more like a Lamborghini, so maybe I&rsquo;ll throw in some airbags and
> it&rsquo;s gonna be mostly up to the driver to protect themselves and drive it safely.

Mental Outlaw on YouTube

Yas really doesn&rsquo;t try, and doesn&rsquo;t want to be the most secure option, but instead rather the fastest (as in no added security delay) or the **bloat free** option (if you ignore the tui version .-.).

Yas also doesn&rsquo;t use pam like any other software that priv escalates, i personnaly see this as a positive, because yas doesn&rsquo;t depend on it.

