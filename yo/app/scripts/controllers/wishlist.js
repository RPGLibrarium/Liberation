'use strict';

/**
 * @ngdoc function
 * @name liberationApp.controller:WishlistCtrl
 * @description
 * # WishlistCtrl
 * Controller of the liberationApp
 */
angular.module('liberationApp')
  .controller('WishlistCtrl', function ($scope) {
    $scope.awesomeThings = [
      'HTML5 Boilerplate',
      'AngularJS',
      'Karma'
    ];
  });
