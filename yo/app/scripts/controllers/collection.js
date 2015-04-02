'use strict';

/**
 * @ngdoc function
 * @name liberationApp.controller:CollectionCtrl
 * @description
 * # CollectionCtrl
 * Controller of the liberationApp
 */
angular.module('liberationApp')
  .controller('CollectionCtrl', function ($scope) {
  	var buch1 = {
  		title:"Wege der Helden", 
  		system:"DSA 4.1", 
  		publisher:"Ulisses Spiele", 
  		owner:"Yoann Kehler",
  		price: 33
  	}
  	var buch2 = {
  		title:"Wege der Helden", 
  		system:"DSA 4.1", 
  		publisher:"Ulisses Spiele", 
  		owner:"Yoann Kehler",
  		price: 44
  	}
    $scope.books = [
      buch1,
      buch2 
    ];
  });
