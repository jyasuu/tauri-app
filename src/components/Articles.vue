<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';

// Define the Article type
interface Article {
  id: number;
  title: string;
  url: string;
  image_url: string;
  news_site: string;
  summary: string;
  published_at: string;
  updated_at: string;
  featured: boolean;
  launches: Array<any>;
  events: Array<any>;
}

// Define the ApiResponse type
interface ApiResponse {
  count: number;
  next: string | null;
  previous: string | null;
  results: Article[];
}

const articles = ref<Article[]>([]);
const count = ref(0);
const next = ref<string | null>(null);
const previous = ref<string | null>(null);
const limit = ref(10);
const offset = ref(0);

const fetchArticles = async (currentOffset = 0) => {
  try {
    const data = await invoke<ApiResponse>('fetch_articles', { limit: limit.value, offset: currentOffset });

    count.value = data.count;
    next.value = data.next;
    previous.value = data.previous;
    articles.value = data.results;
    offset.value = currentOffset;
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
        <button @click="fetchArticles(offset - limit)">Previous</button>
      </div>
      <div v-if="next">
        <button @click="fetchArticles(offset + limit)">Next</button>
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
  