'use strict';

/**
 * @ngdoc function
 * @name liberationApp.controller:MasterpageCtrl
 * @description
 * # MasterpageCtrl
 * Controller of the liberationApp
 */
angular.module('liberationApp')
  .controller('MasterpageCtrl', function ($scope) {
  	//Status of top navigation, if collapsed
  	$scope.navbarMain = {isCollapsed: true};

  	//Status of menuentry "Mein Konto"
  	$scope.btnMeinKonto = {isopen: false};

  	$scope.loginStatus = false;
  });