const std = @import("std");
const Allocator = std.mem.Allocator;

pub const CmdName = enum {
    @"01",
    root,

    pub fn getCmdNameList(alloc: Allocator) ![]const u8 {
        var result = std.ArrayList(u8).init(alloc);
        inline for (@typeInfo(CmdName).Enum.fields) |field| {
            if (field.value == 0) continue;
            try result.appendSlice(field.name);
            try result.append(' ');
        }

        return result.toOwnedSlice(); // Return only the filled portion of the array
    }
};
pub const HistoryType = enum { lenght, area, mian };

pub const ArgValue = union(enum) {
    str: ?[]const u8,
    bool: ?bool,
    num: ?i32,

    pub fn free(self: *ArgValue, allocator: Allocator) !void {
        switch (self.*) {
            .str => |str| if (str) |s| allocator.free(s),
            else => {},
        }
    }
};
pub const Arg = struct {
    long: []const u8,
    short: []const u8,
    info: []const u8,
    value: ArgValue,
    is_alloc: bool = false,
};
pub const ArgError = error{};

pub const ArgsList = std.ArrayList(Arg);

pub fn isHelpOption(opt: []const u8) bool {
    return (std.mem.eql(u8, "-h", opt) or std.mem.eql(u8, "--help", opt));
}
pub fn isVersionOption(opt: []const u8) bool {
    return (std.mem.eql(u8, "-v", opt) or std.mem.eql(u8, "--version", opt));
}
pub const Cmd = struct {
    name: CmdName,
    usage: []const u8,
    example: ?[]const u8 = null,
    info: ?[]const u8 = null,
    min_arg: u8 = 0,
    options: ?[]const Arg = null,
    options_all_sub_cmd: ?[]const Arg = null,
};
const rootCmd = Cmd{
    .name = .root,
    .usage = "zaoc",
    .options = null,
    .info = "This command run's all the dasy for AOC",
    .options_all_sub_cmd = &.{
        .{
            .long = "--part1",
            .short = "-p1",
            .info = "Run part one for the day.",
            .value = .{ .bool = false },
        },
        .{
            .long = "--part2",
            .short = "-p2",
            .info = "Run part two for the day.",
            .value = .{ .bool = false },
        },
        .{
            .long = "--test",
            .short = "-t",
            .info = "Run test for all part for the day.",
            .value = .{ .bool = false },
        },
        .{
            .long = "--test1",
            .short = "-t1",
            .info = "Run test part one for the day.",
            .value = .{ .bool = false },
        },
        .{
            .long = "--test1",
            .short = "-t2",
            .info = "Run test part two for the day.",
            .value = .{ .bool = false },
        },
    },
};
const cmdList: []const Cmd = &.{
    .{
        .name = .@"01",
        .usage = "zaoc 01",
        .info = "This command run's for day 01",
        .options = null,
    },
};

pub const Cli = struct {
    const Self = @This();
    alloc: Allocator,

    name: []const u8,
    description: ?[]const u8 = null,
    process_name: []const u8 = "",

    computed_args: ArgsList,
    subCmds: []const Cmd,
    rootCmd: Cmd,
    cmd: Cmd,
    data: []const u8 = "",

    version: []const u8,

    errorMess: []u8,

    pub fn init(allocate: Allocator, name: []const u8, description: ?[]const u8, version: []const u8) !Self {
        return .{
            .alloc = allocate,
            .name = name,
            .description = description,
            .subCmds = cmdList,
            .rootCmd = rootCmd,
            .cmd = rootCmd,
            .version = version,
            .computed_args = ArgsList.init(allocate),
            .errorMess = try allocate.alloc(u8, 255),
        };
    }
    pub fn parse(self: *Self) !void {
        const args = try std.process.argsAlloc(self.alloc);
        defer std.process.argsFree(self.alloc, args);

        var idx: usize = 1;
        const cmdEnum = if (args.len == 1) CmdName.root else std.meta.stringToEnum(CmdName, args[idx]);
        const cmd = self.getCmd(cmdEnum);

        self.cmd = cmd;
        self.process_name = try self.alloc.dupe(u8, args[0]);
        if (cmd.name != .root) {
            idx += 1;
        }
        if (args.len < idx + cmd.min_arg) {
            std.debug.print("\x1b[1;31m[Error]: Insufficient arguments provided.\x1b[0m\n\n", .{});
            try self.help();
            std.process.exit(0);
        }

        if (self.cmd.options) |opt| {
            idx += try self.parseOptions(opt, idx, args);
        }

        if (cmd.name != .root) {
            if (self.rootCmd.options_all_sub_cmd) |opt| {
                idx += try self.parseOptions(opt, idx, args);
            }
        }

        if (self.cmd.options_all_sub_cmd) |opt| {
            idx += try self.parseOptions(opt, idx, args);
        }

        if (idx >= args.len) return;
        if (isHelpOption(args[idx])) {
            try self.help();
            std.process.exit(0);
        } else if (isVersionOption(args[idx])) {
            std.debug.print("Z Math {s}", .{self.version});
            std.process.exit(0);
        }

        var argList = std.ArrayList(u8).init(self.alloc);
        defer argList.deinit();
        for (args[idx..]) |arg| {
            try argList.appendSlice(arg);
            try argList.append(' ');
        }
        self.data = try argList.toOwnedSlice();
    }

    fn getCmd(self: Self, cmd: ?CmdName) Cmd {
        if (cmd == null) return self.rootCmd;
        for (self.subCmds) |value| {
            if (value.name == cmd) return value;
        }
        return self.rootCmd;
    }
    fn parseOptions(self: *Self, options: []const Arg, arg_idx: usize, args: [][]u8) !usize {
        var idx = arg_idx;
        for (options) |arg| {
            if (idx < args.len and (std.mem.eql(u8, arg.long, args[idx]) or std.mem.eql(u8, arg.short, args[idx]))) {
                var copy_arg = arg;
                switch (arg.value) {
                    .bool => {
                        copy_arg.value = .{ .bool = true };
                        try self.computed_args.append(copy_arg);
                    },
                    .str => {
                        if (idx + 1 >= args.len) {
                            std.debug.print("Error: value reqaired after '{s}'", .{args[idx]});
                            std.process.exit(1);
                        }
                        idx += 1;
                        copy_arg.is_alloc = true;
                        copy_arg.value = .{ .str = try self.alloc.dupe(u8, args[idx]) };
                        try self.computed_args.append(copy_arg);
                    },
                    .num => {
                        if (idx + 1 >= args.len) {
                            std.debug.print("Error: value reqaired after '{s}'", .{args[idx]});
                            std.process.exit(1);
                        }
                        idx += 1;
                        const num = std.fmt.parseInt(i32, args[idx], 10) catch |e| switch (e) {
                            error.InvalidCharacter => null,
                            else => return e,
                        };
                        copy_arg.value = .{ .num = num };
                        try self.computed_args.append(copy_arg);
                    },
                }
                idx += 1;
            } else {
                // NOTE: If an argument has a default value and is not provided,
                // add it to computed_args.
                switch (arg.value) {
                    .bool => |b| if (b != null) try self.computed_args.append(arg),
                    .str => |s| if (s != null) try self.computed_args.append(arg),
                    .num => |n| if (n != null) try self.computed_args.append(arg),
                }
            }
        }
        return idx - arg_idx;
    }
    pub fn getStrArg(self: Self, arg_name: []const u8) !?[]const u8 {
        for (self.computed_args.items) |arg| {
            if (std.mem.eql(u8, arg.long, arg_name) or std.mem.eql(u8, arg.short, arg_name)) {
                if (arg.value != .str) {
                    return error.ArgIsNotStr;
                }
                return arg.value.str;
            }
        }
        return null;
    }

    pub fn getNumArg(self: Self, arg_name: []const u8) !?i32 {
        for (self.computed_args.items) |arg| {
            if (std.mem.eql(u8, arg.long, arg_name) or std.mem.eql(u8, arg.short, arg_name)) {
                if (arg.value != .num) {
                    return error.ArgIsNotNum;
                }
                return arg.value.num;
            }
        }
        return null;
    }
    pub fn getBoolArg(self: Self, arg_name: []const u8) !bool {
        for (self.computed_args.items) |arg| {
            if (std.mem.eql(u8, arg.long, arg_name) or std.mem.eql(u8, arg.short, arg_name)) {
                if (arg.value != .bool) {
                    return error.ArgIsNotBool;
                }
                if (arg.value.bool) |val| {
                    return val;
                } else {
                    return false;
                }
            }
        }
        return false;
    }

    pub fn help(self: Self) !void {
        const padding = 20;
        const stdout = std.io.getStdOut().writer();
        if (self.description) |dis| {
            try stdout.print("Z Math {s}\n{s}\n\n", .{ self.version, dis });
        }
        const cmd_opt = self.cmd;
        try stdout.print("USAGE: \n", .{});
        try stdout.print("  {s}\n\n", .{cmd_opt.usage});
        if (cmd_opt.info) |info| {
            try stdout.print("INFO: \n", .{});
            try stdout.print("  {s}\n\n", .{info});
        }
        if (cmd_opt.example) |ex| {
            try stdout.print("EXAMPLE: \n", .{});
            try stdout.print("  {s}\n\n", .{ex});
        }
        try stdout.print("OPTIONS: \n", .{});
        if (cmd_opt.options) |opt| {
            for (opt) |value| {
                var opt_len: usize = 0;
                opt_len += value.short.len;
                try stdout.print(" {s},", .{value.short});

                opt_len += value.long.len;
                try stdout.print(" {s}", .{value.long});

                for (0..(padding - opt_len)) |_| {
                    try stdout.print(" ", .{});
                }
                try stdout.print("{s}\n", .{value.info});
            }
        }
        if (self.cmd.name != .root) {
            if (self.rootCmd.options_all_sub_cmd) |opt| {
                for (opt) |value| {
                    var opt_len: usize = 0;
                    opt_len += value.short.len;
                    try stdout.print(" {s},", .{value.short});

                    opt_len += value.long.len;
                    try stdout.print(" {s}", .{value.long});

                    for (0..(padding - opt_len)) |_| {
                        try stdout.print(" ", .{});
                    }
                    try stdout.print("{s}\n", .{value.info});
                }
            }
            if (cmd_opt.options_all_sub_cmd) |opt| {
                for (opt) |value| {
                    var opt_len: usize = 0;
                    opt_len += value.short.len;
                    try stdout.print(" {s},", .{value.short});

                    opt_len += value.long.len;
                    try stdout.print(" {s}", .{value.long});

                    for (0..(padding - opt_len)) |_| {
                        try stdout.print(" ", .{});
                    }
                    try stdout.print("{s}\n", .{value.info});
                }
            }
        }
        try stdout.print(" -h, --help            Help message.\n", .{});
        try stdout.print(" -v, --version         App version.\n", .{});
        try stdout.print("\n", .{});
        if (cmd_opt.name != .root) return;
        try stdout.print("COMMANDS: \n", .{});
        for (self.subCmds) |value| {
            if (value.info) |info| {
                const name = @tagName(value.name);
                try stdout.print(" {s}", .{name});
                for (0..(padding - name.len)) |_| {
                    try stdout.print(" ", .{});
                }
                try stdout.print("{s}\n", .{info});
            }
        }
    }
    pub fn deinit(self: *Self) void {
        for (self.computed_args.items) |*item| if (item.is_alloc) try item.value.free(self.alloc);
        self.computed_args.deinit();
        self.alloc.free(self.data);
        self.alloc.free(self.errorMess);
        self.alloc.free(self.process_name);
    }
};

test "Parse Strings" {
    _ = .{
        .{ .{ "--len", "=", "60" }, true },
        .{ .{ "--len", "60" }, true },
        .{ .{ "--len=", "60" }, true },
        .{ .{"--len=60"}, true },
        .{ .{ "--len", "=60" }, true },
        .{ .{"--len"}, true },
        .{ .{"--len60"}, true },
        .{ .{"--len="}, true },
        .{ .{ "--len", "=" }, true },
    };
}
