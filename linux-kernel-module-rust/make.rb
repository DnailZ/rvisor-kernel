#!/usr/bin/env ruby

require "ruby_make_script"
require "syscall.rb"



p syscalls

make do
    :orig_syscall .then do
        # using dir('src/syscall') do
            
        # end
    end
end