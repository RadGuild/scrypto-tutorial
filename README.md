# scrypto-tutorial

Blueprints and code snippets from the Scrypto Tutorial Book

## The Book: First Steps to DeFi: A Scrypto-Tutorial

URL:  https://www.scrypto-tutorial.com

## Authors

Clement Bisaillon

Rock Howard

## Publisher

Radix Programmer's Guild

aka RadGuild on github: https://github.com/RadGuild

## Notes

This tutorial is based on the November 12, 2021 release of Scrypto.

Radix Scrypto on github:  https://github.com/radixdlt/radixdlt-scrypto

## Setup

In order to use the provided Cargo.toml files "as is" in the
provided examples, create the 6 soft links as shown here from
the directories in your current radixdlt-scrypto installation.

```
export scrypto_install=<your-radixdlt-scrypto-install-dir>
ln -s $scrypto_install/radixdlt-scrypto/sbor .
ln -s $scrypto_install/radixdlt-scrypto/sbor-derive .
ln -s $scrypto_install/radixdlt-scrypto/radix-engine .
ln -s $scrypto_install/radixdlt-scrypto/scrypto .
ln -s $scrypto_install/radixdlt-scrypto/scrypto-abi .
ln -s $scrypto_install/radixdlt-scrypto/scrypto-derive .
```

### Optional Setup

Install the Revup tool from https://github.com/RadGuild/revup

You can use the provided ".rev" files to build and run the components.

For example:

```
revup -r Hello_new.rev
```

## Going Forward

The authors intend to update this Tutorial and the code examples as
needed soon after the planned Alexandria release of Scrypto on Dec. 15th, 2021.

## Acknowledgements

The Hello Token and Gumball Machine examples are included in the
basic examples subdirectory within the RadixDLT Scrypto release
and were used as is within the Tutorial.

The authors thank Charlie Cooper and Alanci for corrections.

## Support

This tutorial is and will remain free for any use other than
copying and posting of the contents without acknowledgement.

You can support the authors with tips using XRD or other tokens
on the Radix protocol mainnet.

rdx1qspvz2523hcdfrkkz6kkdd7nej6pgud48qf449wjd4l0dgrzv6tjstcpr9dup

