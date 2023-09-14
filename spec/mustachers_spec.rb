require 'spec_helper'

RSpec.describe Mustachers do
  it 'has a Renderer module' do
    expect(Mustachers).is_a?(Module)
    expect(Mustachers::Renderer).is_a?(Module)
  end

  it 'has a render method' do
    expect(Mustachers::Renderer).to respond_to(:render)
  end

  it 'returns a string' do
    template = '<html lang="{{{lang}}}"/>{{{ top }}}{{{ middle }}}{{{ bottom }}}'

    params = {
      top: '<h1>Page heading</h1>',
      middle: '<p>Paragraph of text.</p>',
      bottom: '<a href="#about">Page link</a>',
      lang: 'en-GB',
      foo:  'bar',
      null_value: nil
    }

    expect(
      Mustachers::Renderer.render(template, params)
    ).to eq('<html lang="en-GB"/><h1>Page heading</h1><p>Paragraph of text.</p><a href="#about">Page link</a>')
  end
end
