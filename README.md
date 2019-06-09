# Introduction

[![Build Status](https://travis-ci.org/bostontrader/bookwerx-core-rust.svg?branch=master)](https://travis-ci.org/bostontrader/bookwerx-core-rust)
[![MIT license](http://img.shields.io/badge/license-MIT-brightgreen.svg)](http://opensource.org/licenses/MIT)

The purpose of **bookwerx-core-rust** is to provide an API that supports multi-currency
 bookkeeping, using the double-entry bookkeeping model, slightly adapted to squeeze 
 in multiple currencies.  It uses [rust](https://www.rust-lang.org), [rocket](https://rocket.rs), and [mysql](https://www.mysql.com).

Any application that deals with "money" (fiat, precious metals, cryptocoins) will
quickly encounter the need for bookkeeping.  Rolling your own methods is, as usual,
 easier said than done, so perhaps you can save yourself some grief and enjoy **bookwerx-core-rust** instead.

With this API, the user can:

* Perform ordinary CRUD operations on the various bookkeeping objects,
such as accounts and transactions.

* Perform consistency checks.


## Getting Started

### Prerequisites

* You will need rust.

* You will need git.

* You will need mysql.

The care and feeding of these items are beyond the scope of these instructions.

### But assuming they are correctly installed...

```bash
git clone https://github.com/bostontrader/bookwerx-core-rust.git
cd bookwerx-core-rust
```
