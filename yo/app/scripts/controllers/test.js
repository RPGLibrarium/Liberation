'use strict';

/**
 * @ngdoc function
 * @name liberationApp.controller:TestCtrl
 * @description
 * # TestCtrl
 * Controller of the liberationApp
 */
angular.module('liberationApp')
  .controller('TestCtrl', function ($scope, Book, $routeParams) {
    $scope.book = Book.get({ id: $routeParams.bookId });
  });
