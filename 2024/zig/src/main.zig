const std = @import("std");
const zarg = @import("./zarg/cmd.zig");

const Allocator = std.mem.Allocator;
const Cli = zarg.Cli;

const day01 = @import("./day/day01.zig");
const day02 = @import("./day/day02.zig");
const Day = @import("./day/day.zig");
const VERSION = "2024+1";
const USAGE =
    \\CLI AOC (Addvent Of Code) App
    \\------------------
    \\A simple CLI AOC (Addvent Of Code) App.
;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer _ = gpa.deinit();
    var cli = try Cli.init(allocator, "Z AOC", USAGE, VERSION);

    defer cli.deinit();
    try cli.parse();
    const dayOpt = Day{
        .t1 = try cli.getBoolArg("-t1"),
        .t2 = try cli.getBoolArg("-t2"),
        .t = try cli.getBoolArg("-t"),
        .p1 = try cli.getBoolArg("-p1"),
        .p2 = try cli.getBoolArg("-p2"),
    };
    switch (cli.cmd.name) {
        .root => {
            const d01 = try day01.init(allocator, dayOpt);
            try d01.run();
            const d02 = try day02.init(allocator, dayOpt);
            try d02.run();
        },
        .@"01" => {
            const d01 = try day01.init(allocator, dayOpt);
            try d01.run();
        },

        .@"02" => {
            const d02 = try day02.init(allocator, dayOpt);
            try d02.run();
        },
        else => {
            std.debug.print("Day {s} not implemented", .{@tagName(cli.cmd.name)});
        },
    }
}
