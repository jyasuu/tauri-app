<script setup lang="ts">
import { ref, onMounted } from 'vue';

const apiUrl = 'https://api.spaceflightnewsapi.net/v4/articles/?limit=10&offset=10';

const articles = ref([]);
const count = ref(0);
const next = ref(null);
const previous = ref(null);

const fetchArticles = async () => {
  try {
    const response = await fetch(apiUrl);
    const data = await response.json();

    count.value = data.count;
    next.value = data.next;
    previous.value = data.previous;
    articles.value = data.results;
  } catch (error) {
    console.error('Error fetching articles:', error);
  }
};

onMounted(() => {
  fetchArticles();
});
</script>

<template>
    <div>
      <h1>Articles</h1>
      <p>Total Articles: {{ count }}</p>
      <div v-if="previous">
        <button @click="fetchArticles(previous)">Previous</button>
      </div>
      <div v-if="next">
        <button @click="fetchArticles(next)">Next</button>
      </div>
      <ul>
        <li v-for="article in articles" :key="article.id">
          <h2>{{ article.title }}</h2>
          <a :href="article.url" target="_blank">Read More</a>
          <img :src="article.image_url" alt="Article Image" />
          <p>{{ article.summary }}</p>
        </li>
      </ul>
    </div>
  </template>
  