$LOAD_PATH << File.join(File.dirname(__FILE__), '../lib')

require 'benchmark'
require 'mustache'
require 'mustachers'

template = '<html lang="{{{lang}}}"/>{{{ top }}}{{{ middle }}}{{{ bottom }}}'

params = {
  top: '<h1>Page heading</h1>',
  middle: '<p>Paragraph of text.</p>',
  bottom: '<a href="#about">Page link</a>',
  lang: 'en-GB',
  foo:  'bar'
}

n = 1_000_000
Benchmark.bm do |x|
  x.report { n.times do; Mustache.render(template, params); end }
  x.report { n.times do; Mustachers::Renderer.render(template, params); end }
end
