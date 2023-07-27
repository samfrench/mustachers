# frozen_string_literal: true

require_relative 'lib/mustachers/version'

Gem::Specification.new do |spec|
  spec.name = 'mustachers'
  spec.version = Mustachers::VERSION
  spec.authors = ['Sam French']

  spec.summary = 'Mustache templating with Rust'
  spec.description = 'Mustache templating with Rust to test performance'
  spec.homepage = 'https://github.com/samfrench/mustachers'
  spec.required_ruby_version = '>= 2.6.0'
  spec.required_rubygems_version = '>= 3.3.11'

  spec.metadata['homepage_uri'] = spec.homepage
  spec.metadata['source_code_uri'] = 'https://github.com/samfrench/mustachers'

  spec.files = Dir.chdir(__dir__) do
    `git ls-files -z`.split("\x0").reject do |f|
      (File.expand_path(f) == __FILE__) ||
        f.start_with?(*%w[bin/ test/ spec/ features/ .git .circleci appveyor Gemfile])
    end
  end
  spec.bindir = 'exe'
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ['lib']
  spec.extensions = ['ext/mustachers/Cargo.toml']

  spec.add_development_dependency 'rspec', '~> 3.12'
  spec.add_development_dependency 'mustache', '~> 1.1'
end
