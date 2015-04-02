angular.module('liberationApp').factory('Book', 
	function($resource){
		return $resource('resources/book/:bookId');
	}
);
