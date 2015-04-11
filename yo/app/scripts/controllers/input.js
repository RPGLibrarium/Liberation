'use strict';

/**
 * @ngdoc function
 * @name liberationApp.controller:InputCtrl
 * @description
 * # InputCtrl
 * Controller of the liberationApp
 */
angular.module('liberationApp')
  .controller('InputCtrl', function ($scope) {

    $scope.newBook = {title: 'Wege des Schwerts', author: 'Ulisses', isbn: '1234'};

    $scope.setTitle = function() {
    	$scope.newBook.title = $scope.formNewTitle;
    	$scope.formNewTitle = '';
    }

    $scope.setAuthor = function() {
    	$scope.newBook.author = formNewAuthor;
    	$scope.newAuthor = '';
    }

    $scope.setIsbn = function() {
    	$scope.newBook.isbn = formNewIsbn;
    	$scope.newIsbn = '';
    }

    $scope.submitToServer = function() {
    	//Implement Restservice here.
    }
  });
