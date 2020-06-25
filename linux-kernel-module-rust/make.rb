#!/usr/bin/env ruby

require "ruby_make_script"
require "./syscall.rb"

c_header = """
#include "hook_syscall.h"
#include <linux/module.h>
#include <linux/init.h>
#include <linux/types.h>
#include <linux/delay.h>
#include <linux/sched.h>
#include <linux/version.h>
#include <linux/kallsyms.h>
#include <linux/semaphore.h>
#include <asm/cacheflush.h>
#include <linux/bitops.h>
#include <linux/sizes.h>
#include <linux/byteorder/generic.h>
#include <linux/preempt.h>
"""


make do
    :orig_syscall .then do
        using dir('src/syscall') do
            File.open('orig.rs', 'w') do |f|
                f.puts "use {"
                f.puts "    super::*,"
                f.puts "}"
                
                f.puts "mod cshim {"
                f.puts "    extern \"C\" {"
                $syscall.each do |k, v|

                end
                f.puts "    }"
                f.puts "}"
            end
        end
    end
end