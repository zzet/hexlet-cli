/* global module */

module.exports = function(grunt) {
  "use strict";
  grunt.loadNpmTasks("grunt-karma");
  grunt.loadNpmTasks("grunt-contrib-requirejs");

  grunt.initConfig({
    pkg: grunt.file.readJSON("package.json"),

    requirejs: {
      compile: {
        options: {
          optimize: "none",
          baseUrl: "./",

          appDir: "js",
          dir: "dist",
          modules: [
            // FIXME
          ]
        }
      }
    },

    karma: {
      unit: {
        configFile: "karma.conf.js",
        autoWatch: true
      }
    }
  });
};
