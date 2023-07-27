# frozen_string_literal: true

require "bundler/gem_tasks"
require "rb_sys/extensiontask"

task build: :compile

RbSys::ExtensionTask.new("mustachers") do |ext|
  ext.lib_dir = "lib/mustachers"
end

task default: :compile
