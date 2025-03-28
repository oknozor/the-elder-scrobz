(function() {
  if (window.parent && window.parent.oidcUserManager) {
    console.log('Processing silent renew callback');
    window.parent.oidcUserManager.signinSilentCallback()
      .then(function() {
        console.log('Silent renew completed successfully');
      })
      .catch(function(error) {
        console.error('Silent renew error:', error);
      });
  } else {
    console.error('Unable to access UserManager from parent window');
  }
})();
