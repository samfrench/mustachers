# Mustachers

Providing mustache rendering using Rust by wrapping the mustache crate.

At the moment only a simple render function taking a template file and parameters to interpolate. This does not look to replace or compete with the ruby offering, but provides an alternative seemingly offering a performance enhancement.

## Installation

Install the gem and add to the application's Gemfile by executing:

    $ bundle add mustachers

If bundler is not being used to manage dependencies, install the gem by executing:

    $ gem install mustachers

## Usage

Require the gem if needed. Some might be dynamically loaded.
```rb
require 'mustachers'

```

An example of using the gem.
```rb
Mustachers::Renderer.render("<h1>{{text}}</h1>", {text: "abc123"})
```

From the console.
```sh
bundle exec ruby -r mustachers -e 'puts Mustachers::Renderer.render("<h1>{{text}}</h1>", {text: "abc123"})'
<h1>abc123</h1>
```

## Benchmark
The script for this can be found in the [benchmarks directory](./benchmarks/simple.rb).
```sh
bundle exec ruby benchmarks/simple.rb
       user     system      total        real
 236.928979   1.059659 237.988638 (239.572824) # ruby
  13.929279   0.069823  13.999102 ( 14.088116) # rust
```

## Development

TODO:
- [ ] Check Ruby performance such as any memory leaks and garbage collection.
- [ ] Improve the error handling to raise an appropriate error.
- [ ] Unit tests in rust to provide faster feedback while developing.
- [ ] More options for the template rendering such as reading the file.
- [ ] Benchmarking with different sized inputs.

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/samfrench/mustachers.
