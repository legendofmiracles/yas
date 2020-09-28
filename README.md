
# Table of Contents

1.  [What is this?](#org545d7b3)
2.  [Installation](#org212c0f0)
    1.  [AUR](#orgcd95674)
    2.  [crates.io (rusts package manager)](#org2ebe32d)
    3.  [From the release page](#orgbbcbf60)
3.  [Usage:](#org8945750)
4.  [Security](#org4bfd103)

yas - a (kind of) sudo replacement
(entire readme is still work in progress)


<a id="org545d7b3"></a>

# What is this?

yas, or &rsquo;yet another sudo&rsquo;, is kind of a sudo replacement, written in rust and without really any configuration options or not needed features.
It is intended for single user systems only, as it works out of the box, with no configuration needed, but you also can&rsquo;t restrict a user from running stuff as root.
yas also can&rsquo;t run commands as any other user, but root.
In case you really do have a user, that shouldn&rsquo;t be allowed to run stuff as root, this isn&rsquo;t for you.


<a id="org212c0f0"></a>

# Installation


<a id="orgcd95674"></a>

## AUR

There is a AUR package for the git version of yas, you can install it with a AUR helper, or just git clone it manually:

    git clone ssh://aur@aur.archlinux.org/yas-git.git
    cd yas-git
    makepkg -is

or:

    yay -S yas-git


<a id="org2ebe32d"></a>

## crates.io (rusts package manager)

Get it from crates.io with rusts cargo, by running:

    cargo install yas

NOTE: The suid bit will not be set, you will manually have to set it for yas to work:

    su -c "chown root ~/.cargo/bin/yas && chmod u+s ~/.cargo/bin/yas"


<a id="orgbbcbf60"></a>

## From the release page

1.  Download a binary
2.  Change the owner to root

    chmod root yas

1.  Set the suid bit

    chmod u+s yas

1.  Put it somewhere in your path, such as /usr/bin


<a id="org8945750"></a>

# Usage:

    yas - execute commands as the root user
    
    usage: yas [-h/--help] [-v/--version ]<command> <arguments for the command, this can be chained infinite>

As one can see, there aren&rsquo;t really any options for yas.
99% of the people who use sudo (or opendoas), just use it with default configuration and the default options.
Who would also need a flag, to edit a file, if nobody ever uses that, and instead just runs \`sudo <editor> <file>\`


<a id="org4bfd103"></a>

# Security

> you could have a car that is very safe by just limiting its
> max speed to 20 miles an hour but I don&rsquo;t want to do that I want to have
> something that&rsquo;s more like a Lamborghini so maybe I&rsquo;ll throw in some airbags and
> it&rsquo;s gonna be mostly up to the driver to protect themselves and drive it safely

yas will ask for the password whenever you want to run a command privileged, unless you ran a command with yas in the last 5 minutes. Not however, that this is cross terminal, but i am planning on fixing that.

yas doesn&rsquo;t ship with so many security features, as sudo, so if you want the most secure root commmand executor, you should also use sudo.

