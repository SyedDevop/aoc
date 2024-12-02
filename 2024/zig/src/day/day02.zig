const std = @import("std");
const Day = @import("./day.zig");

const Allocator = std.mem.Allocator;
const assert = @import("../assert/assert.zig").assert;

const test_input =
    \\7 6 4 2 1
    \\1 2 7 8 9
    \\9 7 6 2 1
    \\1 3 2 4 5
    \\8 6 4 4 1
    \\1 3 6 7 9
;
const test_ans = 2;
const test_ans_2 = 4;

const m_input = @embedFile("../input/input02.txt");

const Self = @This();

alloc: Allocator,
opt: Day,

pub fn init(alloc: Allocator, opt: Day) !Self {
    return .{
        .alloc = alloc,
        .opt = opt,
    };
}
pub fn run(self: Self) !void {
    std.debug.print("Day 02 \n", .{});
    if (self.opt.t1) {
        try self.part01(false);
    } else if (self.opt.t2) {
        try self.part02(false);
    } else if (self.opt.t) {
        try self.part01(false);
        try self.part02(false);
    } else if (self.opt.p1) {
        try self.part01(true);
    } else if (self.opt.p2) {
        try self.part02(true);
    } else {
        try self.part01(true);
        try self.part02(true);
    }
    std.debug.print("\n", .{});
}

fn part01(_: Self, main_input: bool) !void {
    const data = if (main_input) m_input else test_input;
    var lines = std.mem.splitScalar(u8, data, '\n');
    var ans: u64 = 0;
    while (lines.next()) |line| {
        ans += try is_safe(line);
    }
    if (main_input) {
        std.debug.print(" Main part 1 :Ans: |{d}|\n", .{ans});
        return;
    }
    printTest("1", ans, test_ans);
}

fn part02(self: Self, main_input: bool) !void {
    const data = if (main_input) m_input else test_input;
    var lines = std.mem.splitScalar(u8, data, '\n');
    var ans: u64 = 0;
    while (lines.next()) |line| {
        ans += try self.is_real_safe(line);
    }
    if (main_input) {
        std.debug.print(" Main part 2 :Ans: |{d}|\n", .{ans});
        return;
    }
    printTest("2", ans, test_ans_2);
}
fn is_real_safe(self: Self, line: []const u8) !u16 {
    var levels = std.mem.tokenizeScalar(u8, line, ' ');
    const one = std.fmt.parseInt(i32, levels.next() orelse return 0, 10) catch 0;
    const two = std.fmt.parseInt(i32, levels.next() orelse return 0, 10) catch 0;
    const level_increase = one > two;
    levels.reset();
    var i: usize = 0;
    while (levels.next()) |level| {
        i += 1;
        if (level_increase) {
            const onei = std.fmt.parseInt(i32, level, 10) catch 0;
            const twoi = std.fmt.parseInt(i32, levels.peek() orelse break, 10) catch 0;
            const diff = twoi - onei;
            // std.debug.print("DInck diff {d} one {d} tow {d}\n", .{ diff, onei, twoi });
            if (diff < -3 or diff > -1 or diff == 0) {
                const head: []const u8 = levels.buffer[0..i];
                const tail: []const u8 = levels.buffer[2 + i ..];
                var newarry = std.ArrayList(u8).init(self.alloc);
                try newarry.appendSlice(head);
                try newarry.appendSlice(tail);
                const newLevel = try newarry.toOwnedSlice();
                defer self.alloc.free(newLevel);
                return is_safe(newLevel);
            }
        } else {
            const onei = std.fmt.parseInt(i32, level, 10) catch 0;
            const twoi = std.fmt.parseInt(i32, levels.peek() orelse break, 10) catch 0;
            const diff = twoi - onei;
            // std.debug.print("Inck diff {d} one {d} tow {d}\n", .{ diff, onei, twoi });
            if (diff > 3 or diff < 1 or diff == 0) {
                const head: []const u8 = levels.buffer[0..i];
                const tail: []const u8 = levels.buffer[2 + i ..];
                var newarry = std.ArrayList(u8).init(self.alloc);
                try newarry.appendSlice(head);
                try newarry.appendSlice(tail);
                const newLevel = try newarry.toOwnedSlice();
                defer self.alloc.free(newLevel);
                return is_safe(newLevel);
            }
        }
    }
    return 1;
}
fn is_safe(line: []const u8) !u16 {
    var levels = std.mem.tokenizeScalar(u8, line, ' ');
    const one = std.fmt.parseInt(i32, levels.next() orelse return 0, 10) catch 0;
    const two = std.fmt.parseInt(i32, levels.next() orelse return 0, 10) catch 0;
    const level_increase = one > two;
    levels.reset();
    while (levels.next()) |level| {
        if (level_increase) {
            const onei = std.fmt.parseInt(i32, level, 10) catch 0;
            const twoi = std.fmt.parseInt(i32, levels.peek() orelse break, 10) catch 0;
            const diff = twoi - onei;
            // std.debug.print("DInck diff {d} one {d} tow {d}\n", .{ diff, onei, twoi });
            if (diff < -3 or diff > -1 or diff == 0)
                return 0;
        } else {
            const onei = std.fmt.parseInt(i32, level, 10) catch 0;
            const twoi = std.fmt.parseInt(i32, levels.peek() orelse break, 10) catch 0;
            const diff = twoi - onei;
            // std.debug.print("Inck diff {d} one {d} tow {d}\n", .{ diff, onei, twoi });
            if (diff > 3 or diff < 1 or diff == 0)
                return 0;
        }
    }
    return 1;
}

fn printTest(part: []const u8, got: u64, ex: u64) void {
    if (ex == got) {
        std.debug.print("\x1b[38;5;46m [✔] \x1b[0m", .{});
        std.debug.print(" Test part {s} :Ans: |{d}| :Exp: |{d}|\n", .{ part, got, ex });
    } else {
        std.debug.print("\x1b[38;5;196m [✘] \x1b[0m", .{});
        std.debug.print(" Test part {s} :Ans: |{d}| :Exp: |{d}|\n", .{ part, got, ex });
    }
}
