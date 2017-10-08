# extract-domain


# Usage

## Print root domain name


```
$ extract-domain 'hoge.example.com'
example.com
$ extract-domain 'http://hoge.example.com'
example.com
$ extract-domain 'http://anekos@example.com/meow?query=value'
example.com
$ extract-domain 'moge.example.co.jp'
example.co.jp
$ extract-domain 'hoge.moge.example.co.jp'
example.co.jp
~/project/extract-domain$
```
