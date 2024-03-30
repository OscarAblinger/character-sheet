const std = @import("std");
const pretty = @import("pretty.zig");
const testing = std.testing;

export fn add(a: i32, b: i32) i32 {
    return a + b;
}

test "basic add functionality" {
    try testing.expect(add(3, 7) == 10);
}

// ===================================

const Result = union {
    i: i64,
    f: f64,
};

fn getResult() Result {
    return Result{ .i = 115 };
}

test "enum tests" {
    const result = getResult();
    const f = result.f;

    pretty.print(std.heap.c_allocator, f, .{});
    pretty.print(std.heap.c_allocator, result, .{});
    try testing.expect(f == 115.0);
}
