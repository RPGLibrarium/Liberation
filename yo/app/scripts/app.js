'use strict';

/**
 * @ngdoc overview
 * @name liberationApp
 * @description
 * # liberationApp
 *
 * Main module of the application.
 */
angular
  .module('liberationApp', [
    'ngAnimate',
    'ngCookies',
    'ngMessages',
    'ngResource',
    'ngRoute',
    'ngSanitize',
    'ngTouch'
  ])
  .config(function ($routeProvider) {
    $routeProvider
      .when('/collection', {
        templateUrl: 'views/collection.html',
        controller: 'CollectionCtrl'
      })
      .when('/wishlist', {
        templateUrl: 'views/wishlist.html',
        controller: 'WishlistCtrl'
      })
      .when('/account', {
        templateUrl: 'views/account.html',
        controller: 'AccountCtrl'
      })
      .when('/administration', {
        templateUrl: 'views/administration.html',
        controller: 'AdministrationCtrl'
      })
      .otherwise({
        redirectTo: '/'
      });
  });
