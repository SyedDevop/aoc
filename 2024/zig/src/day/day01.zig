const std = @import("std");
const Day = @import("./day.zig");

const Allocator = std.mem.Allocator;
const assert = @import("../assert/assert.zig").assert;

const test_input =
    \\3   4
    \\4   3
    \\2   5
    \\1   3
    \\3   9
    \\3   3
;
const test_ans = 11;
const test_ans_2 = 31;

const m_input = @embedFile("../input/input01.txt");

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
    std.debug.print("Day 01 \n", .{});
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

fn part01(self: Self, main_input: bool) !void {
    const data = if (main_input) m_input else test_input;
    const map = try self.getMap(data);
    defer {
        self.alloc.free(map.l);
        self.alloc.free(map.r);
    }
    var ans: u64 = 0;
    for (0..map.l.len) |i| {
        ans += @intCast(@abs(map.l[i] - map.r[i]));
    }

    if (main_input) {
        std.debug.print(" Main part 1 :Ans: |{d}|\n", .{ans});
        return;
    }
    printTest("1", ans, test_ans);
}

fn part02(self: Self, main_input: bool) !void {
    const data = if (main_input) m_input else test_input;
    const map = try self.getMap(data);
    defer {
        self.alloc.free(map.l);
        self.alloc.free(map.r);
    }
    var ans: u64 = 0;
    var rhash = std.AutoHashMap(i64, i64).init(self.alloc);
    defer rhash.deinit();
    for (map.r) |i| {
        if (rhash.get(i)) |v| {
            try rhash.put(i, v + 1);
        } else {
            try rhash.put(i, 1);
        }
    }
    for (map.l) |i| {
        if (rhash.get(i)) |v| {
            ans += @intCast(v * i);
        }
    }
    if (main_input) {
        std.debug.print(" Main part 2 :Ans: |{d}|\n", .{ans});
        return;
    }
    printTest("2", ans, test_ans_2);
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

const Maps = struct {
    l: []i64,
    r: []i64,
};
fn getMap(self: Self, data: []const u8) !Maps {
    var line = std.mem.splitScalar(u8, data, '\n');
    var lf = std.ArrayList(i64).init(self.alloc);
    var rf = std.ArrayList(i64).init(self.alloc);
    while (line.next()) |l| {
        var nn = std.mem.splitSequence(u8, l, "   ");
        const an = nn.next() orelse "0";
        const bn = nn.next() orelse "0";
        const a: i64 = std.fmt.parseInt(i64, an, 10) catch 0;
        const b: i64 = std.fmt.parseInt(i64, bn, 10) catch 0;
        try lf.append(a);
        try rf.append(b);
    }
    const lmap = try lf.toOwnedSlice();
    const rmap = try rf.toOwnedSlice();
    assert(lmap.len == rmap.len, "The two maps are not equal");
    std.mem.sort(i64, lmap, {}, comptime std.sort.asc(i64));
    std.mem.sort(i64, rmap, {}, comptime std.sort.asc(i64));
    return .{
        .l = lmap,
        .r = rmap,
    };
}
