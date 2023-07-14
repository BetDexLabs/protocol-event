module.exports = {
    verbose: false,
    testMatch: ["**/tests/client/*.ts?(x)"],
    testPathIgnorePatterns: [ "tests/setup.ts", "tests/util/*"],
    globalSetup: "<rootDir>/tests/setup.ts",
    setupFilesAfterEnv: ["<rootDir>/tests/util/test_util.ts"],
    testTimeout: 10000,
};
