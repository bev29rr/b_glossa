document.addEventListener('DOMContentLoaded', () => {
    const content = document.getElementById('content');
    
    if (content) {
      // Start the fade-in animation
      content.classList.remove('hidden');
      content.classList.add('fade-in');
    } else {
      console.error('Element with ID "content" not found.');
    }
  });