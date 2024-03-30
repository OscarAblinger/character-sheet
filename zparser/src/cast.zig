const std = @import("std");
const ArrayList = std.ArrayList;

const CAST = struct {
    nodes: ArrayList(CASTNode),
    model: CASTModel,
    references: ArrayList(CASTReference),

    pub fn deinit(self: *CAST) void {
        self.nodes.deinit();
        self.model.deinit();
        self.references.deinit();
    }
};

const CASTNode = struct {
    startOffset: i32,
    endOffset: i32,
    // todo: fmt
};

const CASTReference = struct {
    name: []u8,
};

const CASTModel = struct {
    featurse: ArrayList(CASTFeature),
};

const CASTFeature = struct {
    name: []u8,
    description: []u8,
    modifiers: ArrayList(CASTModifier),
};

const CASTModifier = struct {
    referencing: *CASTReference,
    value: CASTModifierValue,
};

const CASTModifierValue = union(enum) { SimpleBonus: i32, Bonus: i32, Set: i32 };
